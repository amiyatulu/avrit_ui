import React, { useState, useContext, useEffect } from "react"
import { NearContext } from "../../commons/context/NearContext"
import { useParams, Link } from "react-router-dom"
import GetProductStake from "./GetProductStake"
import PTimeConditionRender from "./PTimeConditionRender"

function ProductData() {
  const { nearvar } = useContext(NearContext)
  const { pid } = useParams()
  return (
    <React.Fragment>
      <GetProductStake pid={pid}/>
      <PTimeConditionRender pid={pid}/>
    </React.Fragment>
  )
}

export default ProductData
