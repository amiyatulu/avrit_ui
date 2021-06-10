import React, { useState } from "react"
import FetchProductScroll from "./FetchProductScroll"
import ProductDetails from "./ProductDetails"

function FetchProductsPage() {
  let startpage = 5
  let endpage = 3
  let { loading, error, productsData, hasMore } = FetchProductScroll(
    startpage,
    endpage
  )


  return (
    <React.Fragment>
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
    </React.Fragment>
  )
}

export default FetchProductsPage
