import React, { Component } from "react";
import Flip from 'react-reveal/Flip';


export class About extends Component {
  render() {
    return (
      <section className="page-section bg-primary" id="about">
        <div className="container">
        <Flip left>
          <div className="row justify-content-center">
            <div className="col-lg-8 text-center">
              <h2 className="text-white mt-0">Abstract</h2>
              <hr className="divider light my-4" />
              <p className="text-white-50 mb-4 text-justify">
              Avrit is a decentralized education system to be built on the top blockchain that evaluates evidence of learning and quality of curriculum; provides a collaboration platform for teachers, students, researchers, and rentable educational resource providers.  It relies on game-theoretic incentive systems where participants earn for the quality contribution they make to the platform. 
              </p>
              <a className="btn btn-light btn-xl js-scroll-trigger" href="paper/AvritWhitePaper.pdf">
                Whitepaper
              </a>
            </div>
          </div>
          </Flip>
        </div>
      </section>
    );
  }
}

export default About;
