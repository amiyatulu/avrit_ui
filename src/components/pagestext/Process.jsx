import React, { useState } from "react"
import "./ContentBestPractices.css"

import schellinggame from "../image/schellinggame.png"

function Process() {
  const [count, setCount] = useState(0)
  return (
    <React.Fragment>
        <br/>
      <div className="container">
        <h2 className="p-3 mb-2 bg-primary text-white">
          Let's walk-through Avrit
        </h2>
        <article className="jumbotron">
          <h3>Login into the account</h3>
          <p>
            Create an account if it doesn't exist. To create an account in
            mainnet you need some near tokens, you can buy near from different
            exchanges like binance, Huobi Global or WazirX. If you already have
            an account, log in using it.
          </p>
        </article>
        <article className="jumbotron">
          <p>
            First, create a profile using profile link. You won't be able to
            stake tokens without creating your profile.
          </p>
        </article>
        <article className="jumbotron">
          <h3>Content Provider:</h3>
          <p>
            After creating your profile go to My Products and create a product.
          </p>
        </article>
        <article className="jumbotron">
          <h3>Reviewer:</h3>
          <p>
            If you are a reviewer, review the product by going to the product
            link. To get incentives for your review, you need to stake the
            minimum amount of Avrit tokens.
          </p>
        </article>
        <article className="jumbotron">
          <h3>Jurors:</h3>
          <p>Jurors will quality access the review.</p>
          <h5>Application:</h5>
          <p>
            So, after a review has been staked, a juror can apply as a jury by
            staking. More than the stake, more the chance of becoming a juror.
          </p>
          <h5>Jury selection:</h5>
          <p>
            After application time is over, we can draw the juror who will be
            selected for the jury process. Those who are not selected can unstake
            their tokens.
          </p>
          <h5>Commit Vote:</h5>
          <p>
            A selected juror can now commit their vote, within commit phase
            time.
          </p>
          <h5>Reveal Vote:</h5>
          <p>
            After the commit phase has ended, selected jurors need to reveal
            their votes.
          </p>
        </article>
        <article className="jumbotron">
          <h3>Winner gets incentives:</h3>
          <figure className="figure">
            <img
              src={schellinggame}
              className="figure-img img-fluid rounded"
              alt="pictorial"
            />
          </figure>
          <p>
            If selected jurors win the Schelling game, they will get incentives,
            the reviewer will get incentives if the judgment is in for the
            review. Product too will get incentives if the rating is more than 2
            stars.
          </p>
        </article>
      </div>
    </React.Fragment>
  )
}

export default Process
