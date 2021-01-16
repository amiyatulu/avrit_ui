const IPFS_URL = process.env.NODE_ENV === "production" ? "https://gateway.ipfs.io/ipfs/" : "https://gateway.ipfs.io/ipfs/"
const CONTRACT_NAME = process.env.NODE_ENV === "production" ? "dev-1610804778703-2060432" : "dev-1610804778703-2060432"
export { IPFS_URL, CONTRACT_NAME }