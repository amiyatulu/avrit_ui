import React, { useState, useEffect } from "react"
import { IPFS_URL } from "../../config/configvar"
import axios from "axios"
import { Link } from "react-router-dom"
import "./ProductDetailsHome.css"

function ProductDetailsHome(props) {
  const [ipfsData, setIpfsData] = useState(false)
  const [loading, setLoading] = useState(true)
  const { ipfshash, id } = props

  useEffect(() => {
    async function fetchProductDetails() {
      // console.log("loading", loading)
      const result = await axios(`${IPFS_URL}${ipfshash}`)
      // console.log(result.data)
      setIpfsData(result.data)
      setLoading(false)
    }
    fetchProductDetails()
  }, [ipfshash])
  //  console.log(ipfsData)
  return (
    <React.Fragment>
      {loading && (
        <p className="container">
          Loading
          <span role="img" aria-label="loading">
            ⌛⌛⌛⌛
          </span>
        </p>
      )}
      {ipfsData && (
        <React.Fragment>
          {/* <h3 className={styles.labelstyle}>Headline</h3> */}
          <div className="card-deck">
            <Link to={`/product/${id}`}>
              <div className="card">
                <img
                  src={`${IPFS_URL}${ipfsData.productimage}`}
                  className="card-img-top"
                  alt=""
                />
                <div className="card-body">
                  <h5 className="card-title">{ipfsData.headline}</h5>
                </div>
              </div>
            </Link>
          </div>
          {/* <h3 className={styles.labelstyle}>Introduction</h3>
          <p className={styles.profilepara}>{ipfsData.introduction}</p>
          <h3 className={styles.labelstyle}>Details</h3>
          <p className={styles.profilepara}>{ipfsData.details}</p>
          <h3 className={styles.labelstyle}>Profile Type</h3>
          <p className={styles.profilepara}>{ipfsData.profile_type}</p>
          <h3 className={styles.labelstyle}>Specialization</h3>
          <p className={styles.profilepara}>{ipfsData.specialization}</p>
          <h3 className={styles.labelstyle}>Audience</h3>
          <p className={styles.profilepara}>{ipfsData.audience}</p> */}
        </React.Fragment>
      )}
    </React.Fragment>
  )
}

export default ProductDetailsHome
