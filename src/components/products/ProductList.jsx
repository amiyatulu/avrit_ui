import React, { useState, useContext, useEffect } from "react"
import { NearContext } from "../../commons/context/NearContext"
import axios from "axios"
import { Link, NavLink } from "react-router-dom"
import { IPFS_URL } from "../../config/configvar"
import ProductDetails from "./ProductDetails"
import styles from "../profile/ViewProfile.module.css"

function LoadingOrCreateProduct(props) {
  const { noProduct, fetchError, noProfile } = props
  if (noProfile) {
    return (
      <React.Fragment>
        <div className="text-center">
          <p>First create a profile:</p>
          <Link type="button" className="btn btn-primary" to="createprofile">
            Create Profile
          </Link>
        </div>
      </React.Fragment>
    )
  }
  if (noProduct) {
    return (
      <React.Fragment>
        <div className="text-center">
          <Link
            type="button"
            className="btn btn-primary"
            to="createproducttopics"
          >
            Create Product
          </Link>
        </div>
      </React.Fragment>
    )
  }
  if (fetchError) {
    return <p className="container">{fetchError}</p>
  }
  return (
    <div className="container">
    <div className="text-center">
      <Link
        type="button"
        className="btn btn-primary"
        to={{ pathname: "/createproducttopics" }}
      >
        Create Product
      </Link>
    </div>
    <br />
    <div>
      <h3 className={styles.labelstyle}>Products</h3>
    <p>
      Loading
      <span role="img" aria-label="loading">
        ⌛⌛⌛⌛
      </span>
    </p>
    </div>
    </div>
  )
}

function ProductList(props) {
  const { nearvar, userId } = useContext(NearContext)
  const [productsData, setProductsData] = useState([])
  const [fetchError, setFetchError] = useState(false)
  const [noProduct, setNoProduct] = useState(false)
  const [noProfile, setNoProfile] = useState(false)
  const page = parseInt(props.page)
  //   console.log(userId)

  useEffect(() => {
    async function fetchProducts() {
      setProductsData([])
      if (userId) {
        let data
        const end = page * 5 
        const start = end -5 
        // console.log(end, "end")
        // console.log(start, "start")

        try {
          data = await nearvar.contract.get_products_of_user_id({
            user_id: parseInt(userId),
            start: start,
            end: end,
          })
          // await nearvar.contract.update_product({product_id:1, product_details_hash:"x"})
        //   console.log(data, "data")

        const productPromises =  data.map(async (x) => {
            // console.log(x)
            return nearvar.contract.get_product({ product_id: x })
                // console.log(hash)                                  
          })

          Promise.all(productPromises).then(hash => {
            //  console.log(hash)
            setProductsData(hash)
        })
        } catch (e) {
          console.error(e.message)
          const errorboolean = e.message.includes("No products for user")
          setNoProduct(errorboolean)
          const errornoprofile = e.message.includes(
            "User profile does not exists"
          )
          setNoProfile(errornoprofile)
          const failedtofetch = e.message
          setFetchError(failedtofetch)
        }
      }
    }

    fetchProducts()
  }, [page, nearvar, userId])
  return (
    <React.Fragment>
      {productsData.length > 0 ? (
        <div className="container">
          <div className="text-center">
            <Link
              type="button"
              className="btn btn-primary"
              to={{ pathname: "/createproducttopics" }}
            >
              Create Product
            </Link>
          </div>
          <br />
          <div>
            <h3 className={styles.labelstyle}>Products</h3>
            <ul className="list-group">
              {productsData &&
                productsData.map((data) => (
                  <React.Fragment key={data.product_id}>
                    <ProductDetails
                      ipfshash={data.product_details_hash}
                      id={data.product_id}
                    />
                  </React.Fragment>
                ))}
            </ul>
          </div>
        </div>
      ) : (
        <LoadingOrCreateProduct
          noProduct={noProduct}
          fetchError={fetchError}
          noProfile={noProfile}
        />
      )}
    </React.Fragment>
  )
}

export default ProductList
