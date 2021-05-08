import React, { useState, useContext} from "react"
import * as Yup from "yup"
import { Formik, Form, Field } from "formik"
import { useHistory, useParams  } from "react-router-dom"
import ipfs from "../../commons/ipfs"
import { NearContext } from "../../commons/context/NearContext"
import { FocusError, SubmittingWheel } from "../../commons/FocusWheel"





function DrawJuror(props) {
    // const [count, setCount] = useState(0);
    const { rid } = useParams()
    console.log(rid, "rid")
    let history = useHistory();
    let {nearvar} = useContext(NearContext)
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
              await nearvar.contract.draw_jurors({
                review_id: rid.toString(),
              }, 95000000000000, 0)
              actions.setSubmitting(false)
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
              <p>3 Jurors are selected per call.</p>
              <div className="text-center">
                <button type="submit" className="btn btn-primary" disabled={isSubmitting}>
                  Submit Form
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



export default DrawJuror

