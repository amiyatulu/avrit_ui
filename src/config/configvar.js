const IPFS_URL = process.env.NODE_ENV === "production" ? "https://gateway.ipfs.io/ipfs/" : "https://gateway.ipfs.io/ipfs/"
const CONTRACT_NAME = process.env.NODE_ENV === "production" ? "dev-1606112714773-6156187" : "dev-1606112714773-6156187"
export { IPFS_URL, CONTRACT_NAME }