import React, { useState, useContext, useEffect } from "react"
import { NearContext } from "../context/NearContext"
import axios from "axios"
import { Link, NavLink } from "react-router-dom"
import { IPFS_URL } from "../config/configvar"

function GetProducts() {
  const nearcontract = useContext(NearContext)
  const [productsData, setProductsData] = useState([])

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
          setProductsData(oldProducts => [...oldProducts, hash])

        })
      } catch (e) {
        console.error(e)
      }
    }

    fetchProducts()
  }, [nearcontract])
  return (
    <React.Fragment>
      <div className="container">
        <p>Hello World!</p>
  <div>{JSON.stringify(productsData)}</div>
      </div>
    </React.Fragment>
  )
}

export default GetProducts
