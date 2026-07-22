use rusqlite::{params, Connection};

pub fn create_tables(conn: &Connection) -> Result<(), rusqlite::Error> {
    conn.execute_batch(
        "
        CREATE TABLE IF NOT EXISTS commands (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title TEXT NOT NULL,
            answer TEXT NOT NULL,
            category TEXT NOT NULL,
            tags TEXT,
            source_url TEXT,
            usage_count INTEGER DEFAULT 0,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP
        );

        CREATE INDEX IF NOT EXISTS idx_commands_category ON commands(category);
        CREATE INDEX IF NOT EXISTS idx_commands_title ON commands(title);
        ",
    )?;
    Ok(())
}

pub fn seed_data_if_empty(conn: &Connection) -> Result<(), rusqlite::Error> {
    let count: i64 = conn.query_row(
        "SELECT COUNT(*) FROM commands",
        [],
        |row| row.get(0),
    )?;

    if count > 0 {
        return Ok(());
    }

    let seed_commands = vec![
        // Git commands
        ("Create git branch", "git checkout -b branch-name", "git", "branch,new,create"),
        ("Switch git branch", "git checkout branch-name", "git", "branch,switch,checkout"),
        ("Delete git branch", "git branch -d branch-name", "git", "branch,delete,remove"),
        ("Stage all changes", "git add .", "git", "stage,add,all"),
        ("Unstage file", "git reset HEAD file.txt", "git", "unstage,reset,remove"),
        ("Commit changes", "git commit -m \"message\"", "git", "commit,save,message"),
        ("Amend last commit", "git commit --amend", "git", "amend,fix,commit"),
        ("Undo last commit (keep changes)", "git reset --soft HEAD~1", "git", "undo,reset,commit"),
        ("Undo last commit (discard changes)", "git reset --hard HEAD~1", "git", "undo,reset,hard,commit"),
        ("View git log", "git log --oneline", "git", "log,history,show"),
        ("View git status", "git status", "git", "status,check,show"),
        ("Create git tag", "git tag -a v1.0.0 -m \"message\"", "git", "tag,create,version"),
        ("Push to remote", "git push origin branch-name", "git", "push,remote,upload"),
        ("Pull from remote", "git pull origin branch-name", "git", "pull,remote,download"),
        ("Clone repository", "git clone https://github.com/user/repo.git", "git", "clone,download,repository"),
        ("Stash changes", "git stash", "git", "stash,save,temporary"),
        ("Apply stashed changes", "git stash pop", "git", "stash,apply,restore"),
        ("View diff", "git diff", "git", "diff,changes,compare"),
        ("Merge branch", "git merge branch-name", "git", "merge,combine,branch"),
        ("Rebase branch", "git rebase branch-name", "git", "rebase,rewrite,history"),
        // Docker commands
        ("List running containers", "docker ps", "docker", "list,running,containers"),
        ("List all containers", "docker ps -a", "docker", "list,all,containers"),
        ("Stop container", "docker stop container_name", "docker", "stop,container,halt"),
        ("Remove container", "docker rm container_name", "docker", "remove,delete,container"),
        ("Remove unused images", "docker image prune -a", "docker", "remove,cleanup,images,unused"),
        ("Remove stopped containers", "docker container prune", "docker", "remove,cleanup,containers,stopped"),
        ("Remove unused networks", "docker network prune", "docker", "remove,cleanup,networks,unused"),
        ("Full system cleanup", "docker system prune -a", "docker", "cleanup,remove,all,system"),
        ("Build image from Dockerfile", "docker build -t image_name .", "docker", "build,image,dockerfile"),
        ("Run container", "docker run -d image_name", "docker", "run,start,container"),
        ("Execute command in container", "docker exec -it container_name bash", "docker", "exec,shell,enter,container"),
        ("View container logs", "docker logs container_name", "docker", "logs,view,output"),
        ("List images", "docker images", "docker", "list,images,show"),
        ("Pull image from registry", "docker pull image_name", "docker", "pull,download,image"),
        ("Push image to registry", "docker push image_name", "docker", "push,upload,image"),
        ("View Docker Compose services", "docker-compose ps", "docker", "compose,services,list"),
        ("Start Docker Compose", "docker-compose up -d", "docker", "compose,start,up"),
        ("Stop Docker Compose", "docker-compose down", "docker", "compose,stop,down"),
        ("View disk usage", "docker system df", "docker", "disk,usage,space"),
        // Linux commands
        ("List files", "ls -la", "linux", "list,files,show"),
        ("Change directory", "cd /path/to/dir", "linux", "directory,change,navigate"),
        ("Print working directory", "pwd", "linux", "directory,current,where"),
        ("Create directory", "mkdir directory_name", "linux", "create,directory,folder"),
        ("Remove file", "rm file.txt", "linux", "remove,delete,file"),
        ("Remove directory", "rm -rf directory_name", "linux", "remove,delete,directory"),
        ("Copy file", "cp source.txt dest.txt", "linux", "copy,duplicate,file"),
        ("Move/rename file", "mv old.txt new.txt", "linux", "move,rename,file"),
        ("Find files", "find /path -name \"*.txt\"", "linux", "find,search,locate"),
        ("Search in files", "grep -r \"pattern\" /path", "linux", "search,grep,find,text"),
        ("View file content", "cat file.txt", "linux", "view,show,read,file"),
        ("Edit file with vim", "vim file.txt", "linux", "edit,vim,modify"),
        ("Check disk space", "df -h", "linux", "disk,space,usage"),
        ("Check memory usage", "free -h", "linux", "memory,usage,ram"),
        ("View running processes", "ps aux", "linux", "process,running,show"),
        ("Kill process by ID", "kill -9 PID", "linux", "kill,stop,process"),
        ("View system info", "uname -a", "linux", "system,info,kernel"),
        ("Check network config", "ifconfig", "linux", "network,ip,interface"),
        ("Download file", "wget https://example.com/file", "linux", "download,fetch,url"),
        ("View file permissions", "ls -l file.txt", "linux", "permissions,mode,access"),
        ("Change permissions", "chmod 755 file.txt", "linux", "permissions,change,mode"),
        ("Change owner", "chown user:group file.txt", "linux", "owner,change,user"),
        // JavaScript commands
        ("Array map", "array.map(item => item.property)", "javascript", "array,transform,iterate"),
        ("Array filter", "array.filter(item => item.condition)", "javascript", "array,filter,select"),
        ("Array reduce", "array.reduce((acc, item) => acc + item, 0)", "javascript", "array,reduce,accumulate"),
        ("Array find", "array.find(item => item.id === 1)", "javascript", "array,find,search"),
        ("Object destructuring", "const { prop1, prop2 } = object", "javascript", "object,destructure,extract"),
        ("Array destructuring", "const [first, second] = array", "javascript", "array,destructure,extract"),
        ("Async function", "async function fetchData() { await fetch(url); }", "javascript", "async,await,fetch"),
        ("Promise", "new Promise((resolve, reject) => { resolve(data); })", "javascript", "promise,async,handle"),
        ("Fetch API", "fetch(url).then(res => res.json())", "javascript", "fetch,api,request"),
        ("Spread operator", "...object", "javascript", "spread,expand,rest"),
        ("Template literal", "`Hello ${name}`", "javascript", "template,string,interpolation"),
        ("Default parameters", "function func(param = 'default') {}", "javascript", "default,parameter,function"),
        ("Arrow function", "const func = (param) => { return result; }", "javascript", "arrow,lambda,function"),
        ("Class definition", "class ClassName { constructor() {} }", "javascript", "class,object,define"),
        ("Try-catch", "try { code } catch (error) { handler }", "javascript", "error,handle,try,catch"),
        ("JSON parse", "JSON.parse(jsonString)", "javascript", "json,parse,convert"),
        ("JSON stringify", "JSON.stringify(object)", "javascript", "json,serialize,convert"),
        ("Local storage set", "localStorage.setItem('key', 'value')", "javascript", "storage,local,set"),
        ("Local storage get", "localStorage.getItem('key')", "javascript", "storage,local,get"),
        ("Event listener", "element.addEventListener('click', handler)", "javascript", "event,listen,handle"),
        // React commands
        ("Create functional component", "function Component() { return <div></div>; }", "react", "component,create,functional"),
        ("useState hook", "const [state, setState] = useState(initialValue)", "react", "hook,state,useState"),
        ("useEffect hook", "useEffect(() => { code }, [deps])", "react", "hook,effect,side-effect"),
        ("useContext hook", "const value = useContext(MyContext)", "react", "hook,context,useContext"),
        ("useRef hook", "const ref = useRef(initialValue)", "react", "hook,ref,useRef"),
        ("useMemo hook", "const memoized = useMemo(() => compute(value), [value])", "react", "hook,memo,performance"),
        ("useCallback hook", "const callback = useCallback(() => { code }, [deps])", "react", "hook,callback,memo"),
        ("Conditional rendering", "{condition && <Component />}", "react", "condition,render,if"),
        ("List rendering", "{array.map(item => <Component key={item.id} />)}", "react", "list,map,render,array"),
        ("Props", "function Component({ prop1, prop2 }) {}", "react", "props,pass,data"),
        ("Children prop", "function Layout({ children }) { return <div>{children}</div>; }", "react", "children,nest,wrapper"),
        ("React Router", "<Route path=\"/\" element={<Home />} />", "react", "router,route,navigate"),
        ("Form handling", "const handleSubmit = (e) => { e.preventDefault(); }", "react", "form,submit,handle"),
        ("Controlled input", "<input value={value} onChange={(e) => setValue(e.target.value)} />", "react", "input,controlled,form"),
        ("useReducer hook", "const [state, dispatch] = useReducer(reducer, initial)", "react", "hook,reducer,state"),
        ("Context Provider", "<MyContext.Provider value={value}>{children}</MyContext.Provider>", "react", "context,provider,share"),
        ("Forward ref", "const Component = forwardRef((props, ref) => <div ref={ref} />)", "react", "ref,forward,access"),
        ("Lazy loading", "const LazyComponent = lazy(() => import('./Component'))", "react", "lazy,load,dynamic"),
        ("Suspense", "<Suspense fallback={<Loading />}><Component /></Suspense>", "react", "suspense,loading,fallback"),
        ("Error boundary", "componentDidCatch(error, info) { handleError(error); }", "react", "error,catch,boundary"),
        // SQL commands
        ("Create table", "CREATE TABLE users (id INT PRIMARY KEY, name VARCHAR(100));", "sql", "create,table,define"),
        ("Select data", "SELECT * FROM users WHERE id = 1;", "sql", "select,query,fetch"),
        ("Insert data", "INSERT INTO users (name, email) VALUES ('John', 'john@example.com');", "sql", "insert,add,new"),
        ("Update data", "UPDATE users SET name = 'Jane' WHERE id = 1;", "sql", "update,modify,change"),
        ("Delete data", "DELETE FROM users WHERE id = 1;", "sql", "delete,remove,row"),
        ("Join tables", "SELECT * FROM users JOIN orders ON users.id = orders.user_id;", "sql", "join,combine,relate"),
        ("Left join", "SELECT * FROM users LEFT JOIN orders ON users.id = orders.user_id;", "sql", "left,join,include"),
        ("Group by", "SELECT category, COUNT(*) FROM products GROUP BY category;", "sql", "group,aggregate,count"),
        ("Having clause", "SELECT category, COUNT(*) FROM products GROUP BY category HAVING COUNT(*) > 5;", "sql", "having,filter,aggregate"),
        ("Order by", "SELECT * FROM users ORDER BY name ASC;", "sql", "order,sort,asc"),
        ("Limit results", "SELECT * FROM users LIMIT 10;", "sql", "limit,restrict,page"),
        ("Create index", "CREATE INDEX idx_name ON users(name);", "sql", "index,performance,fast"),
        ("Drop table", "DROP TABLE users;", "sql", "drop,delete,remove,table"),
        ("Alter table", "ALTER TABLE users ADD COLUMN age INT;", "sql", "alter,modify,add,column"),
        ("Subquery", "SELECT * FROM users WHERE id IN (SELECT user_id FROM orders);", "sql", "subquery,nest,select"),
        ("Union", "SELECT name FROM users UNION SELECT name FROM admins;", "sql", "union,combine,merge"),
        ("Transaction", "BEGIN TRANSACTION; INSERT INTO users...; COMMIT;", "sql", "transaction,commit,rollback"),
        ("View", "CREATE VIEW active_users AS SELECT * FROM users WHERE active = 1;", "sql", "view,stored,query"),
    ];

    for (title, answer, category, tags) in seed_commands {
        conn.execute(
            "INSERT INTO commands (title, answer, category, tags) VALUES (?1, ?2, ?3, ?4)",
            params![title, answer, category, tags],
        )?;
    }

    Ok(())
}
