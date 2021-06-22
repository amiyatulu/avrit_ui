const IPFS_URL = process.env.NODE_ENV === "production" ? "https://gateway.ipfs.io/ipfs/" : "https://gateway.ipfs.io/ipfs/"
const CONTRACT_NAME = process.env.NODE_ENV === "production" ? "dev-1623502935883-30182220358099" : "dev-1623502935883-30182220358099"
export { IPFS_URL, CONTRACT_NAME }