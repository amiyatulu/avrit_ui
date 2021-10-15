import React, { useState, useContext, useEffect } from "react"
import { NearContext } from "../../commons/context/NearContext"
import { useParams, Link } from "react-router-dom"
import GetProductStake from "./GetProductStake"

function ProductData() {
  const { nearvar } = useContext(NearContext)
  const { pid } = useParams()
  return (
    <React.Fragment>
      <div className="container">
        <br/><br/>
        <GetProductStake pid={pid} />
        <Link to={`/productdrawincentives/${pid}/`} className="badge badge-secondary mr-3">
          Draw Incentives for Product
        </Link>
      </div>
    </React.Fragment>
  )
}

export default ProductData
