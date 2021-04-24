const IPFS_URL = process.env.NODE_ENV === "production" ? "https://gateway.ipfs.io/ipfs/" : "https://gateway.ipfs.io/ipfs/"
const CONTRACT_NAME = process.env.NODE_ENV === "production" ? "dev-1619240631051-1427078" : "dev-1619240631051-1427078"
export { IPFS_URL, CONTRACT_NAME }