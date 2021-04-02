const IPFS_URL = process.env.NODE_ENV === "production" ? "https://gateway.ipfs.io/ipfs/" : "https://gateway.ipfs.io/ipfs/"
const CONTRACT_NAME = process.env.NODE_ENV === "production" ? "dev-1617347011273-1256138" : "dev-1617347011273-1256138"
export { IPFS_URL, CONTRACT_NAME }