import React, { useState } from "react"
import Pagination from "@material-ui/lab/Pagination"
import ProductList from "./ProductList"
import Grid from "@material-ui/core/Grid"

export default function ProductPagination() {
  const [page, setPage] = React.useState(1)
  const [productExists, setProductExists] = React.useState(true)
  const handleChange = (event, value) => {
    setPage(value)
  }

  return (
    <div>
      <ProductList page={page} setProductExists={setProductExists} />
      {productExists && (
        <div className="container">
          <Grid container justify="center">
            <Pagination count={10} page={page} onChange={handleChange} />
          </Grid>
        </div>
      )}
    </div>
  )
}
