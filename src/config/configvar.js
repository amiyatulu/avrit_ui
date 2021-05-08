const IPFS_URL = process.env.NODE_ENV === "production" ? "https://gateway.ipfs.io/ipfs/" : "https://gateway.ipfs.io/ipfs/"
const CONTRACT_NAME = process.env.NODE_ENV === "production" ? "dev-1620469614508-9600037" : "dev-1620469614508-9600037"
export { IPFS_URL, CONTRACT_NAME }