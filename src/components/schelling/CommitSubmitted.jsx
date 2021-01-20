import React, { useState } from "react"

function CommitSubmitted(props) {
  return (
    <React.Fragment>
      <div className="container">
        <p className="alert alert-warning">
          Please copy the vote and the hex string in safe place if not done
          already. You will need it to reveal the vote.
        </p>
        <p>
          Vote: <br />
          {props.location.state.vote}
        </p>
        <p>
          Hex: <br />
          {props.location.state.hex}
        </p>
      </div>
    </React.Fragment>
  )
}

export default CommitSubmitted
