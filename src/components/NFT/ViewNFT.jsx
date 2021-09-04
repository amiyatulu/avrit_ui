import React, { useState, useContext, useEffect } from "react"
import { NearContext } from "../../commons/context/NearContext"
import ViewNFTDetails from "./ViewNFTDetails"
import styles from "../profile/ViewProfile.module.css"

function ViewNFT() {
  const { nearvar, userId } = useContext(NearContext)
  const [nftdata, setNftdata] = useState(false)
  useEffect(() => {
    async function getNFT() {
      try {
        let data = await nearvar.contract.last_ten_tokens_for_owner({
          user_id: userId.toString(),
        })
        setNftdata(data.reverse())
      } catch (e) {
        console.error(e)
        const nopriceerror = e.message.includes("Price not set")
      }
    }
    if (userId) {
      getNFT()
    }
  }, [nearvar, userId])

  return (
    <React.Fragment>
      <br />
      <div className="container">
      <h3 className={styles.labelstyle}>Your NFTs</h3>
        {nftdata &&
          nftdata.map((data) => (
            <React.Fragment key={data.token_id}>
              <ViewNFTDetails
                ipfshash={data.metadata.title}
                id={data.token_id.split("_").shift()}
                token_id={data.token_id}
              />
            </React.Fragment>
          ))}
      </div>
    </React.Fragment>
  )
}

export default ViewNFT
