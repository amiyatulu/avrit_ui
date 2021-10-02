import React, { useState, useContext} from "react"
import * as Yup from "yup"
import { Formik, Form, Field } from "formik"
import { useHistory, useParams  } from "react-router-dom"
import ipfs from "../../commons/ipfs"
import { NearContext } from "../../commons/context/NearContext"
import { FocusError, SubmittingWheel } from "../../commons/FocusWheel"

function PUnstakeVote(props) {
    // const [count, setCount] = useState(0);
    const { pid } = useParams()
    // console.log(rid, "rid")
    let history = useHistory();
    let {nearvar, userId, reloadBalance} = useContext(NearContext)
    const [errorThrow, setErrorThrow] = useState(false);

    return (
        <React.Fragment>
      <div className="container">
        <Formik 
          initialValues= {{
             }}
          validationSchema = {Yup.object().shape({
            
          })}
          onSubmit={async (values, actions) => {
          	try {
              await nearvar.contract.p_unstaking_non_selected_juror({
                product_id: pid.toString(),
                user_id: userId.toString()
              })
              actions.setSubmitting(false)
              reloadBalance()
              // console.log(data)
              // history.push(`/thankyou${data.mutationoutputname}`)
              history.goBack()
          	} catch (e) {
              console.error(e)
              setErrorThrow(e.message)
            }
            
              
          }}        
        >
         {({ handleSubmit, handleBlur, handleChange, errors, touched, isValid, isSubmitting, values, setFieldValue, validateForm }) => (
             <Form onSubmit={handleSubmit}>
             {errorThrow && <p>{errorThrow}</p>}
              <div className="text-center">
                <button type="submit" className="btn btn-primary" disabled={isSubmitting}>
                  Unstake
                </button>
                
              </div>
              <SubmittingWheel isSubmitting={isSubmitting} />
              <FocusError />
             </Form>
         )}
        </Formik>
        </div>
        </React.Fragment>
    )
}



export default PUnstakeVote

