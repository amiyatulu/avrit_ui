const IPFS_URL = process.env.NODE_ENV === "production" ? "https://gateway.ipfs.io/ipfs/" : "https://gateway.ipfs.io/ipfs/"
const CONTRACT_NAME = process.env.NODE_ENV === "production" ? "dev-1608206019666-6437801" : "dev-1608206019666-6437801"
export { IPFS_URL, CONTRACT_NAME }