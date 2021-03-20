import Layout from '../../components/layout'
import Date from '../../components/date'
import { getAllTaskIds, getTaskData } from '../../lib/tasks'
import Head from 'next/head'
import utilStyles from '../../styles/utils.module.css'

export async function getStaticProps({ params }) {
  const taskData = await getTaskData(params.id)
  return {
    props: {
      taskData
    }
  }
}

export async function getStaticPaths() {
  const paths = getAllTaskIds()
  return {
    paths,
    fallback: false
  }
}

export default function Task({ taskData }) {
    return (
      <Layout>
        <Head>
          <title>{taskData.title}</title>
        </Head>
        <article>
          <h1 className={utilStyles.headingXl}>{taskData.title}</h1>
          <div className={utilStyles.lightText}>
            <Date dateString={taskData.date} />
          </div>
          <div dangerouslySetInnerHTML={{ __html: taskData.contentHtml }} />
        </article>
      </Layout>
    )
  }