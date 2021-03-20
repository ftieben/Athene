import Layout, { siteTitle } from '../components/layout'
import utilStyles from '../styles/utils.module.css'
import Link from 'next/link'
import Date from '../components/date'
import { getTaskList } from '../lib/tasks'

export async function getStaticProps() {
  const allTasksData = await getTaskList()
  return {
    props: {
      allTasksData
    }
  }
}

export default function Home({ allTasksData }) {
  return (
    <Layout home>
      <section className={`${utilStyles.headingMd} ${utilStyles.padding1px}`}>
        <h2 className={utilStyles.headingLg}>Tasks:</h2>
        <ul className={utilStyles.list}>
        {allTasksData.map(({ id, date, title }) => (
            <li className={utilStyles.listItem} key={id}>
              <Link href={`/tasks/${id}`}>
                <a>{title}</a>
              </Link>
              <br />
              <small className={utilStyles.lightText}>
              <Date dateString={date} />
              </small>
            </li>
          ))}
        </ul>
      </section>
    </Layout>
  )
}