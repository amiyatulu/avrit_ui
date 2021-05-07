const IPFS_URL = process.env.NODE_ENV === "production" ? "https://gateway.ipfs.io/ipfs/" : "https://gateway.ipfs.io/ipfs/"
const CONTRACT_NAME = process.env.NODE_ENV === "production" ? "dev-1620209939982-1910087" : "dev-1620209939982-1910087"
export { IPFS_URL, CONTRACT_NAME }