const IPFS_URL = process.env.NODE_ENV === "production" ? "https://gateway.ipfs.io/ipfs/" : "https://gateway.ipfs.io/ipfs/"
const CONTRACT_NAME = process.env.NODE_ENV === "production" ? "avrit.near" : "dev-1633760482372-45627093335905"
export { IPFS_URL, CONTRACT_NAME }
// dev-1625891373202-46622873112534