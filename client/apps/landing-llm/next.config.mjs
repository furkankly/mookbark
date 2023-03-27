import { PHASE_DEVELOPMENT_SERVER } from "next/constants.js";

export default function config(phase) {
  /** @type {import('next').NextConfig} */
  let nextConfig = {
    experimental: {
      serverComponentsExternalPackages: [
        "puppeteer-core",
        "@sparticuz/chromium-min",
      ],
      esmExternals: true,
    },
    transpilePackages: ["ui"],
    async rewrites() {
      return {
        beforeFiles: [
          {
            source: "/dashboard(.*)",
            destination: `${process.env.MAIN_SERVER}/dashboard$1`,
          },
          {
            source: "/api((?!/chat|/ingest).*)",
            destination: `${process.env.MAIN_SERVER}/api$1`,
          },
        ],
      };
    },
  };
  if (phase === PHASE_DEVELOPMENT_SERVER) {
    nextConfig = {
      ...nextConfig,
      crossOrigin: "use-credentials",
      async headers() {
        return [
          {
            // matching all API routes
            source: "/api/:path*",
            headers: [
              { key: "Access-Control-Allow-Credentials", value: "true" },
              {
                key: "Access-Control-Allow-Origin",
                value: "http://localhost:5173",
              },
              {
                key: "Access-Control-Allow-Methods",
                value: "OPTIONS, GET, DELETE, PATCH, POST, PUT",
              },
              {
                key: "Access-Control-Allow-Headers",
                value:
                  "X-CSRF-Token, X-Requested-With, Accept, Accept-Version, Content-Length, Content-MD5, Content-Type, Date, X-Api-Version",
              },
              {
                key: "Access-Control-Request-Headers",
                value: "*",
              },
            ],
          },
        ];
      },
    };
    return nextConfig;
  }
  return nextConfig;
}
