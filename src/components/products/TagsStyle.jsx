import React, { useState } from "react"
import styles from "./Tags.module.css"

function TagsStyle(props) {
  const tags = props.tags
  const links = tags.split(",").map((tag, index) => (
    
      <li key={index}>
        <span className={styles.tag}>
          {tag}
        </span>
      </li>
      
    
  ))
  return <React.Fragment><ul className={styles.tags}>{links}</ul></React.Fragment>
}

export default TagsStyle
