import React, { useState, useContext, useEffect } from "react"
import { NearContext } from "../../commons/context/NearContext"
import { BigNumber } from "bignumber.js"
import WithdrawNFTBalance from "./WithdrawNFTBalance"

function NFTBalance() {
  const { nearvar, userId } = useContext(NearContext)
  const [balance, setBalance] = useState(null)
  let pw = BigNumber(10).pow(18)

  useEffect(() => {
    async function fetchBalance() {
      try {
        let balance = await nearvar.contract.get_owner_incentives({
          user_id: userId.toString(),
        })
        setBalance(balance)
      } catch (e) {
        console.log(e)
      }
    }

    fetchBalance()
  }, [nearvar, userId])
  return (
    <React.Fragment>
      <br />
      <br />
      <div className="container">
        {balance && (
          <React.Fragment>
            <p className="badge badge-secondary mr-3">
              NFT Sell Balance: {BigNumber(balance).div(pw).toFixed()} Avrit
            </p>
          </React.Fragment>
        )}
      </div>
      <div>
          <WithdrawNFTBalance />
      </div>
    </React.Fragment>
  )
}

export default NFTBalance
