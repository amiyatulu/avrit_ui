import React, { useState } from "react"
import styles from "./HomeDetails.module.css"
import selfmanagement from "./image/selfmanagement.png"
import tree from "./image/tree.png"

function HomeDetails() {
  const [count, setCount] = useState(0)
  return (
    <React.Fragment>
      <div className={`${styles.firstdivbackground} fluid-container`}>
        <div className="row">
          <div className="col-lg-6 col-12 p-5 my-auto">
            <h2>Self Management</h2>
            <p>
              Build your teal institutes through self-management, not through
              hierarchical power structures. We make self-management easy by
              giving you a platform for quality checks and collaboration. <br />
              To know more about self-management: <br />
              <a
                href="https://reinventingorganizationswiki.com/theory/self-management/"
                target="_blank"
                rel="noopener noreferrer"
              >
                Reinventing Organization
              </a>
            </p>
            <p>Build institutes that serve everyone, not a few.</p>
          </div>

          <div className={`col-lg-6 col-12 p-0 ${styles.homeimagediv}`}>
            <img src={selfmanagement} alt="Self Management" className="w-100" />
          </div>
        </div>
        <div className="row">
          <div className={`col-lg-6 col-12 p-0 ${styles.homeimagediv}`}>
            <img src={tree} alt="Evolutionary Purpose" className="w-100" />
          </div>

          <div className="col-lg-6 col-12 p-5 my-auto">
            <h2>Evolutionary Purpose</h2>
            <p>
              Learn with an evolutionary purpose that reflects a deeper reason
              and meaning to your learning. Not for the sake of scoring high
              marks. <br/>Grow emotionally, relationally, spiritually, and
              academically with your teacher, family, friends, and society.
            </p>
          </div>
        </div>
        <br />
        <br />
        <br />
      </div>
    </React.Fragment>
  )
}

export default HomeDetails
