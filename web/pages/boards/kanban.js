import Link from 'next/link'
import Layout from '../../components/layout'

import Head from 'next/head'



export default function FirstPost() {
  return (
    <Layout>
      <Head>
        <title>Peter</title>
      </Head>
      <h1>First Post</h1>
      <h2>
        <Link href="/">
          <a>Back to home</a>
        </Link>
        
      </h2>
    </Layout>
  )
}