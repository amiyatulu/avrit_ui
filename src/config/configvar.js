const IPFS_URL = process.env.NODE_ENV === "production" ? "https://gateway.ipfs.io/ipfs/" : "https://gateway.ipfs.io/ipfs/"
const CONTRACT_NAME = process.env.NODE_ENV === "production" ? "dev-1607010890790-1250367" : "dev-1607010890790-1250367"
export { IPFS_URL, CONTRACT_NAME }