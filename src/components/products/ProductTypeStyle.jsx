import React, { useState } from "react"
import ProductById from "./ProductById"
import styles from "./Tags.module.css"

function ProductTypeStyle(props) {
  const ProductType = props.type
  const Productvalues = {
    ev: "Evidence of Learning",
    oa: "Open Access",
    rm: "Room",
    cm: "Curriculum",
    as: "Assignment",
    oh: "Others",
  }
  const fullname = Productvalues[ProductType]
  return (
    <React.Fragment>
      <ul className={styles.tags}>
        <li>
          <span className={styles.tag}>{fullname}</span>
        </li>
      </ul>
    </React.Fragment>
  )
}

export default ProductTypeStyle
