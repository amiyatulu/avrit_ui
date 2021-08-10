import React, { useState } from "react"

function PrivacyPolicy() {
  const [count, setCount] = useState(0)
  return (
    <React.Fragment>
        <div className="container">
            <br/>
      <h2>Privacy Policy </h2>
      <p>
        Avrit recognizes the importance of maintaining your privacy. We value
        your privacy and appreciate your trust in us. We might collect your
        name, email, phone number, and location, and your publicly shared posts.
        We respect your privacy. You can share your posts with your
        pseudo-anonymous account id. We don't collect any information other than
        that.
      </p>
      </div>
    </React.Fragment>
  )
}

export default PrivacyPolicy
