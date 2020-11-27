import React, { useState, useContext, useEffect } from "react"
import { NearContext } from "../context/NearContext"
import axios from "axios"
import { Link, NavLink } from "react-router-dom"
import { IPFS_URL } from "../config/configvar"
import ProductDetails from "./ProductDetails"
import styles from "../profile/ViewProfile.module.css"

function GetProducts() {
  const nearcontract = useContext(NearContext)
  const [productsData, setProductsData] = useState([])
  const [loading, setLoading] = useState(true)

  useEffect(() => {
    async function fetchProducts() {
      let data
      try {
        data = await nearcontract.contract.get_products_of_user()
        // await nearcontract.contract.update_product({product_id:1, product_details_hash:"x"})
        console.log(data)
        setLoading(false)

        data.map(async (x) => {
          console.log(x)
          let hash = await nearcontract.contract.get_product({ product_id: x })
          console.log(hash)
          setProductsData((oldProducts) => [...oldProducts, hash])
        })
      } catch (e) {
        console.error(e)
      }
    }

    fetchProducts()
  }, [nearcontract])
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
      {!loading && (
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
      )}
    </React.Fragment>
  )
}

export default GetProducts
