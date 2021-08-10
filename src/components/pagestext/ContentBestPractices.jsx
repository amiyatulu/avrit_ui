import React, { useState } from "react"
import "./ContentBestPractices.css"
import pictorial from "../image/pictorial.jpg"
import concrete from "../image/concrete.png"
import probing from "../image/probingquestions.png"
import solvedunsolved from "../image/solved_unsolved.png"
import example from "../image/example.png"
import classification from "../image/classification.png"
import feedforward from "../image/feedforwad_loop.png"
import analogy from "../image/analogy.png"
import analogy2 from "../image/analogy2.png"
import family from "../image/family.jpg"
import synthesize from "../image/synthesize_questions.jpg"
import book from "../image/book.jpg"
import scientific from "../image/scientific-methods.png"
import scientificskills from "../image/scientificskills.png"
import plusminus from "../image/plusminus.jpg"
import message from "../image/message.png"
import message2 from "../image/message2.png"
import plantcolonization from "../image/plant_colonization.png"
import abt from "../image/abt.png"
import logic from "../image/logic.png"
import logic2 from "../image/logic2.png"
import whysex from "../image/why-sex.png"
import instrinsicload from "../image/instrinsic_load.png"
import cognitiveload from "../image/cognitive-load.png"
import learningobjectives from "../image/learning_objectives.png"
import reference from "../image/reference.png"


function ContentBestPractices() {
  return (
    <React.Fragment>
      <div className="container">
        <br/>
        <h2 className="p-3 mb-2 bg-primary text-white">Content Best Practices</h2>
        <article className="jumbotron">
          <h3>Pairing graphics with words</h3>
          <p>
            It's about communicating information both verbally and through
            graphics. We, humans, receive information using two pathways,
            auditory and visual. Our learning gets enhanced by pairing graphics
            with words. Graphics include diagrams, flowcharts, visual
            illustrations, animation, or video.
          </p>
          <figure className="figure">
            <img
              src={pictorial}
              className="figure-img img-fluid rounded"
              alt="pictorial"
            />
          </figure>
        </article>
         <article className="jumbotron">
          <h3>Linking abstract concepts with concrete representations</h3>
          <p>
            Abstract ideas can be hard to understand. One example can be dealing
            with fractions. We can represent 1½ in concrete representation as
            one and a half pizza.
          </p>
          <figure className="figure">
            <img
              src={concrete}
              className="figure-img img-fluid rounded"
              alt="concrete"
            />
          </figure>
        </article>
         <article className="jumbotron">
          <h3>Posing probing questions</h3>
          <p>
            People with better questioning skills learn better. Asking probing
            questions like "why", "how", "compare and contrast", what are the
            pieces of evidence, what are the causes and effects, help us to dig
            deeper into the subject, understand mechanisms and bridge the
            knowledge gap.
          </p>
          <figure className="figure">
            <img
              src={probing}
              className="figure-img img-fluid rounded"
              alt="probing"
            />
            <figcaption className="figure-caption text-center">
              <a
                href="http://mrkempnz.com/2014/06/questioning-the-most-powerful-tool-in-the-classroom-an-action-research.html"
                target="_blank"
                rel="noopener noreferrer"
              >
                Questioning – the most powerful tool in the classroom – an
                action research
              </a>
            </figcaption>
          </figure>
        </article>
         <article className="jumbotron">
          <h3>
            Repeatedly alternating problems with their solutions provided and
            problems that students must solve
          </h3>
          <p>
            First, a solved example problem is given, then students are asked to
            solve another similar problem.
          </p>
          <figure className="figure">
            <img
              src={solvedunsolved}
              className="figure-img img-fluid rounded"
              alt="solved unsolved"
            />
            <figcaption className="figure-caption text-center">
              Solved example and unsolved problem with steps of problem-solving
              strategy: 1) sort 2)strategize 3) solve and 4) check. <br />–
              Chemistry, A Molecular Approach by Nivaldo J. Tro
            </figcaption>
          </figure>
        </article>
         <article className="jumbotron">
          <p>
            Statements or ideas or concepts should be backed by adequate
            examples and research data as evidence. Formulas should be provided
            with its implementation examples in physical situations. <br />
            e.g. Have you ever heard the phrase “form follows function?” It’s a
            philosophy practiced in many industries. In architecture, this means
            that buildings should be constructed to support the activities that
            will be carried out inside them. For example, a skyscraper should be
            built with several elevator banks; a hospital should be built so
            that its emergency room is easily accessible.
          </p>
          <blockquote className="blockquote">
            <p className="mb-0">
              One example is not enough, provide as many as you can to explain
              its use in a different context and situations.
            </p>
            {/* <footer className="blockquote-footer">Someone famous in <cite title="Source Title">Source Title</cite></footer> */}
          </blockquote>
          <figure className="figure">
            <img
              src={example}
              className="figure-img img-fluid rounded"
              alt="Example"
            />
            <figcaption className="figure-caption text-center">
              Function and Limits from book Essential Calculus by James Stewart
            </figcaption>
          </figure>
          <blockquote className="blockquote">
            <p className="mb-0">
              Give example questions to classify from several antagonistic
              observations.
            </p>
          </blockquote>
          <p>
            e.g. In probability, a simple event is an outcome that cannot be
            further broken down into simple components. So its example can be:
            <br />
            Roll one die: 5 (simple event)
            <br />
            Rolling one die has 6 simple events: 1, 2, … 6<br />
            Roll two dice: 7 (not a simple event)
            <br />
            Rolling two dice have 36 simple events: 1-1, 1-2,….., 6-6, and 7 is
            NOT a simple event because it CAN BE broken down into simpler
            events, such as 3-4 or 6-1.
          </p>
          <figure className="figure">
            <img
              src={classification}
              className="figure-img img-fluid rounded"
              alt="Classification"
            />
            <figcaption className="figure-caption">
              A classification problem for iris flower from sepal and petal
              dimensions
            </figcaption>
          </figure>
        </article>
         <article className="jumbotron">
          <p>
            Diagrammatic representation of text with proper labeling and
            caption containing enough description should be provided.
          </p>
          <figure className="figure">
            <img
              src={feedforward}
              className="figure-img img-fluid rounded"
              alt="Feedforward"
            />
            <figcaption className="figure-caption">
              Molecular Biology of the Cell, Alberts
            </figcaption>
          </figure>
        </article>
         <article className="jumbotron">
          <p>
            Books should have analogies with the diagram in abundance to
            convert all abstract, new and tough concepts into concrete. It’s an
            important feature, that makes a book exceptionally great and boosts
            learning
          </p>
          <blockquote className="blockquote">
            <p className="mb-0">
              Analogies can be misleading if it’s not too close to the model you
              are explaining. Always provide contrasting characters of analogy
              in comparison to the real model.
            </p>
          </blockquote>
          <p>
            Example of an analogy: Converting AC to DC
            <br />
            The converter uses a diode, a tiny electronic device that acts as a
            one-way valve to allow electron flow in one direction only. Since
            alternating current changes its direction each half-cycle, current
            passes through a diode only half of each period. The output is a
            rough dc, and it is off half the time. To maintain continuous
            current while smoothing the bumps, a capacitor is used. The
            capacitor acts as a storage reservoir for a charge.
          </p>
          <div> Diagram with analogies:</div>
          <figure className="figure">
            <img
              src={analogy}
              className="figure-img img-fluid rounded"
              alt="Analogy"
            />
            <img
              src={analogy2}
              className="figure-img img-fluid rounded"
              alt="Analogy2"
            />

            <figcaption className="figure-caption">
              Conceptual Physics, by Paul G. Hewitt
            </figcaption>
          </figure>
        </article>
         <article className="jumbotron">
          <p>
            Questions should be probing, also many questions should be
            graphics/diagrams with texts.
          </p>
          <figure className="figure">
            <img
              src={family}
              className="figure-img img-fluid rounded"
              alt="Family"
            />
          </figure>
          <figure className="figure">
            <img
              src={synthesize}
              className="figure-img img-fluid rounded"
              alt="synthesize"
            />
            <figcaption className="figure-caption">
              A Question with image from Campbell Biology
            </figcaption>
          </figure>
        </article>
         <article className="jumbotron">
          <p>
            Length of the book should respect the time of students. It
            doesn’t mean book can’t be long or elaborative. Being elaborative is
            not about giving lots of different concepts or topics, it means
            explaining a few topics elaborately so that it requires least effort
            to grasp. A narrow but precise statement can take more time to
            comprehend than elaborative explanations with examples, images, and
            analogies. Each sentence should have a purpose of being in the book.
          </p>

          <blockquote className="blockquote">
            <p className="mb-0">
              Don’t make it (the content) simple, teach the complexity by
              simplifying it. The curriculum should cover the depth of a concept
              and breadth of the subject without the clutter.
            </p>
          </blockquote>

          <figure className="figure">
            <img
              src={book}
              className="figure-img img-fluid rounded"
              alt="book"
            />
          </figure>
        </article>
         <article className="jumbotron">
          <p>
            A book should also provide memory assistance for hard to remember
            information. <br />
            “Karaoke players can order free grape soda” <br />
            Kingdom–phylum–class–order–family–genus–species
          </p>
        </article>
         <article className="jumbotron">
          <p>Posing a probing question before explaining the topic.</p>
          <figure className="figure">
            <img
              src={scientific}
              className="figure-img img-fluid rounded"
              alt="book"
            />
            <figcaption className="figure-caption">
              Conceptual Physical Science by Hewitt, Suchocki, Hewitt
            </figcaption>
          </figure>
        </article>
         <article className="jumbotron">
          <p>
            Content should include questions that address life skills like
            collaboration, and 21st-century skills like the ability to analyze
            and interpret data. It must have questions with a section for data
            interpretation and analysis and questions for group work along with
            conceptual, mathematical and review questions.
          </p>

          <figure className="figure">
            <img
              src={scientificskills}
              className="figure-img img-fluid rounded"
              alt="scientific skills"
            />
            <figcaption className="figure-caption">
              Chemistry a Molecular Approach, Nivaldo J Tro
            </figcaption>
          </figure>
          <p>
            An example of question that promotes critical thinking. Critical
            thinking questions are hard to develop and require a lot of time for
            research and effort:
          </p>
          <figure className="figure">
            <img
              src={plusminus}
              className="figure-img img-fluid rounded"
              alt="plus minus"
            />
          </figure>

          <blockquote className="blockquote">
            <p className="mb-0">
              <a
                href="https://iambrainstorming.wordpress.com/2018/02/17/question-designing-should-be-brain-friendly-cue-based/"
                target="_blank"
                rel="noopener noreferrer"
              >
                Questions should be brain friendly. Memory is reconstructive not
                reproductive.
              </a>
            </p>
          </blockquote>
        </article>
         <article className="jumbotron">
          <p>
            Each section should give a message or lesson about big ideas that
            help to make a judgment or transfer of learning in different
            situations. The essence of the message should be in the title that
            is descriptive and a further descriptive subtitle.
          </p>
          <figure className="figure">
            <img
              src={message}
              className="figure-img img-fluid rounded"
              alt="message"
            />
            <figcaption className="figure-caption">
              A descriptive title from book Molecular Biology of the Cell,
              Alberts
            </figcaption>
          </figure>
          <figure className="figure">
            <img
              src={message2}
              className="figure-img img-fluid rounded"
              alt="message"
            />
            <figcaption className="figure-caption">
              A descriptive subtitle from book Inorganic Chemistry by Shriver,
              Weller, Overton, Rourke and Armstrong
            </figcaption>
          </figure>
          <blockquote className="blockquote">
            <p className="mb-0">
              Is the content clever? <br />
              It’s not adequate to have a comprehending content, its more to do
              with smart content (big ideas, learning it to apply in the real
              world thinking, building connections to link ideas to get the
              complete picture, encourage critical thinking and inquiry) for
              smart teaching and smart learning.
            </p>
          </blockquote>
        </article>
         <article className="jumbotron">
          <p>
            Engaging style of telling is essential in scientific writing. A
            chapter may begin with a narrative/storytelling of scientific
            experiences of researchers and their comments, history and
            progression in the discussed field or example events relevant to the
            subject matter. Even narratives can be used in a separate text box
            to discuss example events or how researchers reach their
            conclusions.
          </p>
          <blockquote className="blockquote">
            <p className="mb-0">
              The narrative technique provides a deeper meaning for the reader
              and helps the reader to use imagination to visualize situations.
            </p>
          </blockquote>
          <figure className="figure">
            <img
              src={plantcolonization}
              className="figure-img img-fluid rounded"
              alt="message"
            />
            <figcaption className="figure-caption">
              Colonization of Plants in the barren landscape Campbell Biology,
              Reece, Urry, Cain et. al.
            </figcaption>
          </figure>

          <blockquote className="blockquote">
            <p className="mb-0">
              But narratives should be used with caution. Anecdotal evidence
              (evidence collected in a casual or informal manner and relying
              heavily or entirely on personal testimony) are not reliable
              evidences.
            </p>
          </blockquote>
          <p>How to make stories with ABT (And, But, Therefore)?</p>

          <blockquote className="blockquote">
            <p className="mb-0">
              There are three main forces involved in creating a story —
              agreement, contradiction, consequence.
            </p>
            <p className="mb-0">ABT: All you need to know to tell stories</p>
            <p>
              <a
                href="https://github.com/amiyatulu/teacher_resources/raw/master/Stories:AndButTherefore.pdf"
                target="_blank"
                rel="noopener noreferrer"
              >
                https://github.com/amiyatulu/teacher_resources
              </a>
            </p>
          </blockquote>
          <figure className="figure">
            <img
              src={abt}
              className="figure-img img-fluid rounded"
              alt="message"
            />
            <figcaption className="figure-caption">
              ABT: All You need to know to tell stories by Randy Olson, i wonder
            </figcaption>
          </figure>

          <blockquote className="blockquote">
            <p className="mb-0">
              The conclusion should be supported by the premises and the
              arguments with contradictions, inconsistencies, and exceptions
              mentioned in the arguments.
            </p>
            <p>
              Extremely simplified models are useful to anyone who understands
              the assumptions, understands how the assumptions drive outcomes,
              understands how violation of assumptions affects outcomes, and
              understands how this defines the scope of the original model and
              its extensions.
            </p>
          </blockquote>
          <div>
            There are three parts to any proposition: <br />
            The premises – i.e. facts which are already known or assumed. <br />
            The argument – i.e. the logical chain of reasoning that leads from
            the premises to the conclusion. <br />
            The conclusion – i.e. that which must be true if the premises are
            true and the argument is valid. <br />
          </div>
          <blockquote className="blockquote">
            <p className="mb-0">
              The conclusion must be true IF the premises are true AND the
              argument is valid.
            </p>
          </blockquote>
          <figure className="figure">
            <img
              src={logic}
              className="figure-img img-fluid rounded"
              alt="message"
            />
            <figcaption className="figure-caption">
              A concise Introduction to Logic by Patrick J. Hurley, Lori Watson
            </figcaption>
          </figure>
          <figure className="figure">
            <img
              src={logic2}
              className="figure-img img-fluid rounded"
              alt="message"
            />
            <figcaption className="figure-caption">
              Without logic, your working long hours will make things even worse
            </figcaption>
          </figure>
        </article>
         <article className="jumbotron">
          <p>
            Topics can have connection boxes containing discussion about
            situations that are surprising, nonintuitive or widely-held false
            ideas.
          </p>

          <figure className="figure">
            <img
              src={whysex}
              className="figure-img img-fluid rounded"
              alt="message"
            />
            <figcaption className="figure-caption">
              Why do males exits? Why bdelloids have completely abandoned sex?
              Biology, the unity and diversity of life, by Starr, Taggart, Evers
              et al.
            </figcaption>
          </figure>
        </article>
         <article className="jumbotron">
          <p>
            Content must be designed to accord with cognitive load theory.
            Cognitive load theory is built upon two commonly accepted ideas. The
            first is that there is a limit to how much new information the human
            brain can process at one time. The second is that there are no known
            limits to how much-stored information can be processed at one time.{" "}
            <br />
            Cognitive overload occurs when the total cognitive load exceeds the
            working memory capacity of the learner.
            <br />
            e.g. This material creates a high intrinsic cognitive load because
            of use of too many technical terminologies (many times hard to
            imagine and comprehend) in a single sentence or paragraph.
          </p>
          <figure className="figure">
            <img
              src={instrinsicload}
              className="figure-img img-fluid rounded"
              alt="message"
            />
            <figcaption className="figure-caption">NCERT Biology</figcaption>
          </figure>
          <p>
            Cognitive load theory in detail:
            <a
              href="https://github.com/amiyatulu/teacher_resources/raw/master/cognitive_load_theory_report_AA1.pdf"
              target="_blank"
              rel="noopener noreferrer"
            >
              Cognitive Load Theory Report
            </a>
          </p>
          <figure className="figure">
            <img
              src={cognitiveload}
              className="figure-img img-fluid rounded"
              alt="message"
            />
            <figcaption className="figure-caption">
              Education Centre for Education Statistics &amp; Evaluation
            </figcaption>
          </figure>
          <blockquote className="blockquote">
            <p className="mb-0">
              New information should be presented slowly after the previous one
              is mastered by elaboration and practice.
            </p>
            <p>
              <a
                href="https://iambrainstorming.blogspot.com/2017/10/practicing-one-component-in-one-time-to.html"
                target="_blank"
                rel="noopener noreferrer"
              >
                Component Practice
              </a>
            </p>
          </blockquote>
        </article>
         <article className="jumbotron">
          <p>
            Books should have learning outcomes or objectives at the beginning
            or end of each chapter. Examples, practice, and exercises that test
            your confidence and expertise in each learning objectives must be
            provided.
          </p>
          <figure className="figure">
            <img
              src={learningobjectives}
              className="figure-img img-fluid rounded"
              alt="pictorial"
            />
            <figcaption className="figure-caption">
              Key Learning Outcomes from Book: Chemistry A Molecular Approach,
              Nivaldo J. Tro
            </figcaption>
          </figure>
          <p>
            Blog on the importance of sharing learning outcomes:
            <a
              href="https://www.learningscientists.org/blog/2017/10/4-1"
              target="_blank"
              rel="noopener noreferrer"
            >
              Should I share my learning outcomes with students?
            </a>
          </p>
        </article>
         <article className="jumbotron">
          <p>
            End of the chapter can provide references to the texts. Referencing
            is a way to provide evidence to support the assertions and claims in
            the content. It also helps the reader to dig deeper into the
            scientific methodology that validates the assertions and claims.
          </p>
          <figure className="figure">
            <img
              src={reference}
              className="figure-img img-fluid rounded"
              alt="pictorial"
            />
            <figcaption className="figure-caption">
            The Psychology of Human Sexuality by Justin J. Lehmiller
            </figcaption>
          </figure>
        </article>
      </div>
    </React.Fragment>
  )
}

export default ContentBestPractices
