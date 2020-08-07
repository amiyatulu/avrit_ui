import React, { useState, useEffect } from "react"
import { IPFS_URL } from "../config/configvar"
import axios from "axios"

function ProductDetails(props) {
  const [ipfsData, setIpfsData] = useState(false)
  const { ipfshash } = props

  useEffect(() => {
    async function fetchProductDetails() {
      const result = await axios(`${IPFS_URL}${ipfshash}`)
      console.log(result.data)
      setIpfsData(result.data)
    }
    fetchProductDetails()
  }, [ipfshash])

  return (
    <React.Fragment>
      {ipfsData && (
        <React.Fragment>
          <h3>Headline</h3>
          <p>{ipfsData.headline}</p>
          <h3>Introduction</h3>
          <p>{ipfsData.introduction}</p>
          <h3>Details</h3>
          <p>{ipfsData.details}</p>
          <h3>Profile Type</h3>
          <p>{ipfsData.profile_type}</p>
          <h3>Specialization</h3>
          <p>{ipfsData.specialization}</p>
          <h3>Audience</h3>
          <p>{ipfsData.audience}</p>
        </React.Fragment>
      )}
    </React.Fragment>
  )
}

export default ProductDetails
