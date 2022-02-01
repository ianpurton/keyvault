import * as k8s from "@pulumi/kubernetes";
import * as kx from "@pulumi/kubernetesx";
import * as pulumi from "@pulumi/pulumi";
import { service, deployment } from './util'

const APP_NAME="app"
const AUTH_NAME="auth"
const WWW_NAME="www"
const ENVOY_NAME="envoy"

const NAME_SPACE = 'cloak'
const DB_URL_SECRET = 'database-urls'
const MIGRATION_DB_URL = 'migrations-database-url'
const APP_DB_URL = 'app-database-url'
const AUTH_DB_URL = 'auth-database-url'
const config = new pulumi.Config();

const ENVOY_IMAGE=`purtontech/cloak-envoy:${config.require('version')}@${config.require('cloak-envoy')}`
const APP_IMAGE=`purtontech/cloak-server:${config.require('version')}@${config.require('cloak-server')}`
const MIGRATIONS_IMAGE=`purtontech/cloak-db-migrations:${config.require('version')}@${config.require('cloak-db-migrations')}`
const WWW_IMAGE=`purtontech/cloak-website:${config.require('version')}@${config.require('cloak-website')}`

const envoyPod = new kx.PodBuilder({
    imagePullSecrets: [{ name: 'image-pull' }],
    containers: [{
        name: "envoy",
        image: ENVOY_IMAGE,
        ports: { http: 7100 }
    }]
})

const cloudflaredPod = new kx.PodBuilder({
    imagePullSecrets: [{ name: 'image-pull' }],
    containers: [{
        name: "tunnel",
        image: "cloudflare/cloudflared:2021.11.0",
        command: ["cloudflared", "tunnel"],
        args: [
            `--url=http://${ENVOY_NAME}:7100`,
            `--hostname=tunnel.cloak.software`,
            "--origincert=/etc/cloudflared/cert.pem",
            "--no-autoupdate"
        ],
        volumeMounts: [{
            name: "tunnel-secret-volume",
            mountPath: "/etc/cloudflared/"
        }],
    }],
    volumes: [{
        name: "tunnel-secret-volume",
        secret: {
            secretName: `cloudflare-cert-${NAME_SPACE}`,
            items: [
                { key: "cert.pem", path: "cert.pem" }
            ]
        }
    }]
})

const appPod = new kx.PodBuilder({
    imagePullSecrets: [{ name: 'image-pull' }],
    containers: [{
        name: APP_NAME,
        image: APP_IMAGE,
        ports: { http: 7103 },
        env: [
            { name: "APP_DATABASE_URL", 
                valueFrom: {
                    secretKeyRef: {
                        name: DB_URL_SECRET,
                        key: APP_DB_URL
                    }
                }
            }
        ]
    }],
    initContainers: [{
        name: "server-init",
        image: MIGRATIONS_IMAGE,
        imagePullPolicy: 'Always',
        env: [
            { name: "DATABASE_URL", 
                valueFrom: {
                    secretKeyRef: {
                        name: DB_URL_SECRET,
                        key: MIGRATION_DB_URL
                    }
                }
            }
        ]
    }]
})

const wwwPod = new kx.PodBuilder({
    imagePullSecrets: [{ name: 'image-pull' }],
    containers: [{
        name: WWW_NAME,
        image: WWW_IMAGE,
        ports: { http: 80 }
    }]
})

const authPod = new kx.PodBuilder({
    imagePullSecrets: [{ name: 'image-pull' }],
    containers: [{
        name: AUTH_NAME,
        image: 'authnproxy/authnproxy:latest',
        imagePullPolicy: 'Always',
        ports: { http: 9090 },
        env: [
            { name: "AUTH_TYPE", value: "encrypted" },
            { name: "DATABASE_URL", 
                valueFrom: {
                    secretKeyRef: {
                        name: DB_URL_SECRET,
                        key: AUTH_DB_URL
                    }
                }
            },
            { name: "SECURE_COOKIE", value: 'true' },
            { name: "REDIRECT_URL", value: '/app/post_registration' },
            { name: "FORWARD_URL", value: '127.0.0.1' }, 
            { name: "FORWARD_PORT", value: '8080' },
            { name: "SKIP_AUTH_FOR", value: '/$$,/images/*,/static/*,/contact' },
            { name: "SECRET_KEY", value: config.requireSecret('secret_key') },
        ]
    }]
})

deployment("cloudflared", cloudflaredPod, NAME_SPACE)
const envoyDeployment = deployment(ENVOY_NAME, envoyPod, NAME_SPACE)
const wwwDeployment = deployment(WWW_NAME, wwwPod, NAME_SPACE)
const authDeployment = deployment(AUTH_NAME, authPod, NAME_SPACE)
const appDeployment = deployment(APP_NAME, appPod, NAME_SPACE)

service(APP_NAME, appDeployment, NAME_SPACE, 7103, 7103)
service(WWW_NAME, wwwDeployment, NAME_SPACE, 7104, 80)
service(AUTH_NAME, authDeployment, NAME_SPACE, 9090, 9090)
service(ENVOY_NAME, envoyDeployment, NAME_SPACE, 7100, 7100)