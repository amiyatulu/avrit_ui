import React, { useState } from "react"
import styles from "./Home.module.css"
import child from "./image/child.svg"
import ball from "./image/ball.svg"
import FetchProductsPage from "./products/FetchProductsPage"
import HomeDetails from "./HomeDetails"

function Home() {
  const [count, setCount] = useState(0)
  return (
    <React.Fragment>
      <div className={`container-fluid`}>
        <div className={`row`}>
          <div className={`col-md-6 ${styles.leftcolumnone}`}></div>
          <div className={`col-md-6 ${styles.rightcolumnone}`}>
            <div className={`row ${styles.rowone} align-items-center `}>
              <div className={`col-8 text-center`}>
                <div className={`${styles.avrittitle}`}>Avrit</div>
                <div className={`${styles.avrittext}`}>
                  Earn for Learning
                </div>
                <div className={`${styles.ball}`}>
                  <img src={ball} alt="ball" height="80px" />
                </div>
              </div>

              <div className={`col-4 text-center ${styles.imagecss}`}>
                <img src={child} alt="child" />
              </div>
            </div>
            <div className={`row ${styles.rowtwo} align-items-center`}>
              <div className={`col ${styles.avritdescription}`}>
                <h3>Learning the Hard Way</h3>
                Learning can be fun, but it requires perseverance and practice
                to master a subject. Lessons are not meant to be easy, but
                challenging, structured with evidence-based learning strategies,
                relevant and achievable. Join Avrit, mentor your students, kids,
                siblings, or friends to become independent learners. Write the
                curriculum, review, upload evidence of learning, do your
                projects, and earn cryptocurrency.
              </div>
            </div>
          </div>
        </div>
        <br/>
        <br/>
      
        <div className={styles.fetchproduct}>
            <HomeDetails/>
        <h3 className="text-center"> Products </h3>
        <FetchProductsPage />
        </div>
      </div>
     
    </React.Fragment>
  )
}

export default Home
