const IPFS_URL = process.env.NODE_ENV === "production" ? "https://gateway.ipfs.io/ipfs/" : "https://gateway.ipfs.io/ipfs/"
const CONTRACT_NAME = process.env.NODE_ENV === "production" ? "dev-1596733083520-5954561" : "dev-1596733083520-5954561"
export { IPFS_URL, CONTRACT_NAME }