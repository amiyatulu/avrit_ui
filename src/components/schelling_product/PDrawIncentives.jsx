import React, { useState, useContext} from "react"
import * as Yup from "yup"
import { Formik, Form, Field } from "formik"
import { useHistory, useParams  } from "react-router-dom"
import ipfs from "../../commons/ipfs"
import { NearContext } from "../../commons/context/NearContext"
import { FocusError, SubmittingWheel } from "../../commons/FocusWheel"





function PDrawIncentives(props) {
    // const [count, setCount] = useState(0);
    const { rid } = useParams()
    // console.log(rid, "rid")
    let history = useHistory();
    let {nearvar, reloadBalance} = useContext(NearContext)
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
              await nearvar.contract.p_incentives_distribution({
                product_id: rid.toString(),
              })
              actions.setSubmitting(false)
              // console.log(data)
              // history.push(`/thankyou${data.mutationoutputname}`)
              reloadBalance()
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
                  Draw Incentives
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



export default PDrawIncentives

