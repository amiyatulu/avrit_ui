import React, { useState, useContext, useEffect } from "react"
import { NearContext } from "../../commons/context/NearContext"
import axios from "axios"
import { Link, NavLink } from "react-router-dom"
import { IPFS_URL } from "../../config/configvar"
import ProductDetails from "./ProductDetails"
import styles from "../profile/ViewProfile.module.css"


function LoadingOrCreateProduct(props){
  const {noProduct, fetchError, noProfile} = props
  if(noProfile) {
    return(
      <React.Fragment>
        <div className="text-center">
          <p>First create a profile:</p>
          <Link type="button" className="btn btn-primary" to="createprofile" >
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
  const {nearvar} = useContext(NearContext)
  const [productsData, setProductsData] = useState([])
  const [fetchError, setFetchError] = useState(false)
  const [noProduct, setNoProduct] = useState(false)
  const [noProfile, setNoProfile] = useState(false)

  useEffect(() => {
    async function fetchProducts() {
      let data
      try {
        data = await nearvar.contract.get_products_of_user({start:0, end:5})
        // await nearvar.contract.update_product({product_id:1, product_details_hash:"x"})
        console.log(data)

        data.map(async (x) => {
          console.log(x)
          let hash = await nearvar.contract.get_product({ product_id: x })
          console.log(hash)
          setProductsData((oldProducts) => [...oldProducts, hash])
        })
      } catch (e) {
        console.error(e.message)
        const errorboolean = e.message.includes("No products for user")
        setNoProduct(errorboolean)
        const errornoprofile = e.message.includes("User profile does not exists")
        setNoProfile(errornoprofile)
        const failedtofetch = e.message
        setFetchError(failedtofetch)
      }
    }

    fetchProducts()
  }, [nearvar])
  return (
    <React.Fragment>
      {productsData.length > 0 ? (
        <div className="container">
          <div className="text-center">
            <Link type="button" className="btn btn-primary" to={{ pathname:"/createproduct"}}>
            Create Product
          </Link>
          </div>
          <br/>
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
        <LoadingOrCreateProduct noProduct={noProduct} fetchError={fetchError}  noProfile={noProfile}/>
      )}
    </React.Fragment>
  )
}

export default GetProducts
