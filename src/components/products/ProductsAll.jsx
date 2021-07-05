import React, { useState } from "react"
import FetchProductsPage from "./FetchProductsPage"

function ProductsAll() {
  return (
    <React.Fragment>
      <div className="container">
        <br />
        <FetchProductsPage />
      </div>
    </React.Fragment>
  )
}

export default ProductsAll
