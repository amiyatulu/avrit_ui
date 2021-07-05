import { PrivateKey } from '@textile/hub'

const version = 10003 //Math.floor(Math.random() * 1000);
const IDENTITY_KEY = 'identity-' + version


export const generateIdentity = () => {
    let idStr = localStorage.getItem(IDENTITY_KEY)
    if (idStr) {
      return PrivateKey.fromString(idStr)
    } else {
      const id = PrivateKey.fromRandom()
      idStr = id.toString()
      localStorage.setItem(IDENTITY_KEY, idStr)
      return id
    }
  }