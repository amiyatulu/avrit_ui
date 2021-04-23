const IPFS_URL = process.env.NODE_ENV === "production" ? "https://gateway.ipfs.io/ipfs/" : "https://gateway.ipfs.io/ipfs/"
const CONTRACT_NAME = process.env.NODE_ENV === "production" ? "dev-1619096055674-2617195" : "dev-1619096055674-2617195"
export { IPFS_URL, CONTRACT_NAME }