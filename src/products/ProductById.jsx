import React, { useState, useContext, useEffect } from "react"
import { NearContext } from "../context/NearContext"
import { useParams } from "react-router-dom"
import axios from "axios"
import styles from "../profile/ViewProfile.module.css"
import { IPFS_URL } from "../config/configvar"

function ProductById() {
  const nearcontract = useContext(NearContext)
  const { id } = useParams()
  const [ipfsData, setProductData] = useState(null)

  useEffect(() => {
    async function fetchProduct() {
      try {
        let data = await nearcontract.contract.get_product({
          product_id: parseInt(id),
        })
        console.log(data)
        const result = await axios(`${IPFS_URL}${data.product_details_hash}`)
        console.log(result.data)
        setProductData(result.data)
      } catch (e) {
        console.error(e)
      }
    }

    fetchProduct()
  }, [nearcontract, id])

  return (
    <React.Fragment>
      {ipfsData && (
        <div className="container">
          <h3 className={styles.labelstyle}>Headline</h3>
          <p className={styles.profilepara}>{ipfsData.headline}</p>
          <h3 className={styles.labelstyle}>Introduction</h3>
          <p className={styles.profilepara}>{ipfsData.introduction}</p>
          <h3 className={styles.labelstyle}>Details</h3>
          <p className={styles.profilepara}>{ipfsData.details}</p>
          <h3 className={styles.labelstyle}>Specialization</h3>
          <p className={styles.profilepara}>{ipfsData.specialization}</p>
          <h3 className={styles.labelstyle}>Audience</h3>
          <p className={styles.profilepara}>{ipfsData.audience}</p>
        </div>
      )}
    </React.Fragment>
  )
}

export default ProductById
