import React, { useState } from "react"
import { Link } from "react-router-dom"

import styles from "./CreateProductTopics.module.css"
import "./ProductTopics.css"

function CreateProductTopics() {
  const [count, setCount] = useState(0)

  return (
    <React.Fragment>
      <div className={`container`}>
        <div className="row">
          <div className="col-md-5 col-sm-6 content-card">
            <div className="card-big-shadow">
              <Link to="/createproduct/ev/">
                <div
                  className="card card-just-text"
                  data-background="color"
                  data-color="blue"
                  data-radius="none"
                >
                  <div className="content">
                    {/* <h6 className="category">Best cards</h6>
                    <h4 className="title"><a href="#">Blue Card</a></h4> */}
                    <h2 className="title">Evidence of Learning</h2>
                    <p class="description">Upload your evidence of learning to earn incentives</p>
                    
                  </div>
                </div>
              </Link>
            </div>
          </div>
          <div class="col-md-5 col-sm-6 content-card">
            <div class="card-big-shadow">
              <Link to="/createproduct/oa/">
                <div
                  class="card card-just-text"
                  data-background="color"
                  data-color="green"
                  data-radius="none"
                >
                  <div class="content">
                    {/* <h6 class="category">Best cards</h6>
                    <h4 class="title"><a href="#">Green Card</a></h4> */}
                    <h2 class="title">Open Access</h2>
                    <p class="description">Write open access content to help students and earn incentives</p>
                  </div>
                </div>
              </Link>
            </div>
          </div>

          <div class="col-md-5 col-sm-6 content-card">
            <div class="card-big-shadow">
              <Link to="/createproduct/cm/">
                <div
                  class="card card-just-text"
                  data-background="color"
                  data-color="yellow"
                  data-radius="none"
                >
                  <div class="content">
                    {/* <h6 class="category">Best cards</h6> */}
                    <h2 class="title">Curriculum</h2>
                   
                    <p class="description">Design the curriculum for students</p>
                  </div>
                </div>
              </Link>
            </div>
          </div>

          <div class="col-md-5 col-sm-6 content-card">
            <div class="card-big-shadow">
              <Link to="/createproduct/as/">
                <div
                  class="card card-just-text"
                  data-background="color"
                  data-color="brown"
                  data-radius="none"
                >
                  <div class="content">
                    <h2 class="title">Assignment</h2>
                    <p class="description">
                    Upload your assignments and projects here
                  </p>
                  </div>
                </div>
              </Link>
            </div>
          </div>

          {/* <div class="col-md-5 col-sm-6 content-card">
            <div class="card-big-shadow">
              <Link to="/createproduct/rm/">
                <div
                  class="card card-just-text"
                  data-background="color"
                  data-color="purple"
                  data-radius="none"
                >
                  <div class="content">
                    <h2 class="title">Room</h2>
                    <p class="description">
                    If you are providing rooms for studies, give the details here
                  </p>
                  </div>
                </div>
              </Link>
            </div>
          </div> */}

          {/* <div class="col-md-5 col-sm-6 content-card">
            <div class="card-big-shadow">
              <Link to="/createproduct/oh/">
                <div
                  class="card card-just-text"
                  data-background="color"
                  data-color="orange"
                  data-radius="none"
                >
                  <div class="content">
                    <h2 class="title">Others</h2>
                  </div>
                </div>
              </Link>
            </div>
          </div> */}
        </div>
      </div>
    </React.Fragment>
  )
}

export default CreateProductTopics
