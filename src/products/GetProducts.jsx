import React, { useState, useContext, useEffect } from "react"
import { NearContext } from "../context/NearContext"
import axios from "axios"
import { Link, NavLink } from "react-router-dom"
import { IPFS_URL } from "../config/configvar"
import ProductDetails from "./ProductDetails"
import styles from "../profile/ViewProfile.module.css"


function LoadingOrCreateProduct(props){
  const {noProduct, fetchError} = props
  if (noProduct) {
    return (
      <React.Fragment>
        <div className="text-center">
          <Link type="button" className="btn btn-primary" to="createproduct" >
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
    <p className="container">
      Loading
      <span role="img" aria-label="loading">
        ⌛⌛⌛⌛
      </span>
    </p>
  )
}



function GetProducts() {
  const nearcontract = useContext(NearContext)
  const [productsData, setProductsData] = useState([])
  const [fetchError, setFetchError] = useState(false)
  const [noProduct, setNoProduct] = useState(false)

  useEffect(() => {
    async function fetchProducts() {
      let data
      try {
        data = await nearcontract.contract.get_products_of_user()
        // await nearcontract.contract.update_product({product_id:1, product_details_hash:"x"})
        console.log(data)

        data.map(async (x) => {
          console.log(x)
          let hash = await nearcontract.contract.get_product({ product_id: x })
          console.log(hash)
          setProductsData((oldProducts) => [...oldProducts, hash])
        })
      } catch (e) {
        console.error(e.message)
        const errorboolean = e.message.includes("No products for user")
        setNoProduct(errorboolean)
        const failedtofetch = e.message
        setFetchError(failedtofetch)
      }
    }

    fetchProducts()
  }, [nearcontract])
  return (
    <React.Fragment>
      {productsData ? (
        <div className="container">
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
      ): (
        <LoadingOrCreateProduct noProduct={noProduct} fetchError={fetchError}/>
      )}
    </React.Fragment>
  )
}

export default GetProducts
