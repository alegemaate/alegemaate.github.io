import React from "react"

import Layout from "../components/layout"
import Image from "../components/image"
import SEO from "../components/seo"

import ExternalLink from "../components/externalLink"
import ContactItem from "../components/contactItem"

import {
  faGamepad,
  faVolumeUp,
  faEnvelope,
} from "@fortawesome/free-solid-svg-icons"
import { faGithub, faSoundcloud } from "@fortawesome/free-brands-svg-icons"

const IndexPage = ({ data }) => (
  <Layout>
    <SEO title="About Allan Legemaate" />
    <div className="wrapper">
      <div className="box flex-center">
        <h1>Allan Legemaate</h1>
      </div>

      <div className="box">
        <Image />
      </div>

      <div className="box ">
        <h2>Who am I?</h2>
        <p>
          I am a 3rd year Computer Science student studying at Queen's
          University.
        </p>

        <h2>What do I do?</h2>
        <p>
          I am currently working as a full stack developer at Innovasium
          Digital. In my spare time I make games and produce music.
        </p>

        <h2>Contact me</h2>
        <ContactItem text="alegemaate@gmail.com" icon={faEnvelope} />
        <div className="smallSpace" />
        <div className="smallSpace" />
      </div>

      <div className="box">
        <h2>My Pages</h2>
        <ExternalLink
          text="github"
          icon={faGithub}
          src="https://github.com/alegemaate"
        />
        <div className="smallSpace" />
        <ExternalLink
          text="soundcloud"
          icon={faSoundcloud}
          src="https://soundcloud.com/allan-legemaate"
        />
        <div className="smallSpace" />
        <ExternalLink
          text="freesound"
          icon={faVolumeUp}
          src="https://freesound.org/people/alegemaate/"
        />
        <div className="smallSpace" />
        <ExternalLink
          text="adsgames"
          icon={faGamepad}
          src="https://adsgames.net"
        />
        <div className="smallSpace" />
      </div>
    </div>
  </Layout>
)

export default IndexPage
