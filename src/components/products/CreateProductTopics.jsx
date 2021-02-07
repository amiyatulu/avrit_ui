import React, { useState } from "react"
import { Link } from "react-router-dom"

import styles from "./CreateProductTopics.module.css"

function CreateProductTopics() {
  const [count, setCount] = useState(0)

  return (
    <React.Fragment>
      <div className={`container`}>
        <div className="row">
          <div className={`col-5 ${styles.gutter}`}>
            <Link to="/createproduct/ev/">
              <div className={`p-3 border ${styles.box} ${styles.cont}`}>
                Evidence of Learning
              </div>
            </Link>
          </div>

          <div className={`col-5 ${styles.gutter}`}>
            <Link to="/createproduct/oa/">
              <div className={`p-3 border ${styles.box} ${styles.cont}`}>
                Open Access
              </div>
            </Link>
          </div>

          <div className={`col-5 ${styles.gutter}`}>
            <Link to="/createproduct/cm/">
              <div className={`p-3 border ${styles.box} ${styles.cont}`}>
                Curriculum
              </div>
            </Link>
          </div>

          <div className={`col-5 ${styles.gutter}`}>
            <Link to="/createproduct/as/">
              <div className={`p-3 border ${styles.box} ${styles.cont}`}>
                Assignment
              </div>
            </Link>
          </div>

          <div className={`col-5 ${styles.gutter}`}>
            <Link to="/createproduct/rm/">
              <div className={`p-3 border ${styles.box} ${styles.cont}`}>
                Room
              </div>
            </Link>
          </div>

          <div className={`col-5 ${styles.gutter}`}>
            <Link to="/createproduct/oh/">
              <div className={`p-3 border ${styles.box} ${styles.cont}`}>
                Others
              </div>
            </Link>
          </div>
        </div>
      </div>
    </React.Fragment>
  )
}

export default CreateProductTopics
