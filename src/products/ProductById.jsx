import React, { useState, useContext, useEffect } from "react"
import { NearContext } from "../context/NearContext"
import { useParams, Link } from "react-router-dom"
import axios from "axios"
import styles from "../profile/ViewProfile.module.css"
import { IPFS_URL } from "../config/configvar"
import GetReviews from '../reviews/GetReviews'

function ProductById() {
  const nearcontract = useContext(NearContext)
  const { id } = useParams()
  const [ipfsData, setProductData] = useState(null)
  const [productType, setProductType] = useState(null)

  useEffect(() => {
    async function fetchProduct() {
      try {
        let data = await nearcontract.contract.get_product({
          product_id: parseInt(id),
        })
        console.log(data)
        setProductType(data.product_type)
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
          <div className="jumbotron">
            <h3 className="display-4">{ipfsData.headline}</h3>
            <h5>Introduction:</h5>
            <p>{ipfsData.introduction}</p>
            <h5>Details:</h5>
            <p>{ipfsData.details}</p>
            <h5>Specialization:</h5>
            <p>{ipfsData.specialization}</p>
            <h5>Audience:</h5>
            <p>{ipfsData.audience}</p>
            <h5>Product Type:</h5>
            <p>{productType}</p>
            <Link
              to={`/createreview/${id}`}
              className="badge badge-secondary mr-3"
            >
              Create Review
            </Link>
          </div>
          <div>
          <GetReviews pid={id} />
          </div>
        </div>
      )}
    </React.Fragment>
  )
}

export default ProductById
