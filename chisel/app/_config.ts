interface Config {
  SERVER_API_ENDPOINT: string;
  GOOGLE_CLIENT_ID: string;
  NEXTJS_API_URL: string;
}
export default {
  SERVER_API_ENDPOINT: process.env.SERVER_API_ENDPOINT,
  GOOGLE_CLIENT_ID: process.env.NEXT_PUBLIC_GOOGLE_CLIENT_ID,
  NEXTJS_API_URL: process.env.NEXT_PUBLIC_API_ENDPOINT,
} as Config;
