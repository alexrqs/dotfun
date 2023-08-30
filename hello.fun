fun hello () {
  foo mutable = "good"
  .log "hello how are you ${foo}"
  .error "hello how is the error"

  if true == true

  else

  end
  
  :apple

  add = a, b => a + b
  
  add 1, 2

  bar = {
    now: "hello"
    far: "bye"
  }

  case bar do
    {now} when now == "hello" => 
      "match"
    {far} when far == "bye" =>
      "boo"
  end
  
  const isProd = process.env.NODE_ENV === 'production'

  app.use express.json {limit: '3mb'}
  app.use > express.json > { limit: '3mb' }
  app.use(express.json({ limit: '3mb' })) // for parsing application/json

  app.use express.urlencoded { limit: '3mb' }
  app.use > express.urlencoded > { extended: true }
  app.use(express.urlencoded({ extended: true })) // for parsing application/x-www-form-urlencoded

  conditions do
    (2 + 2 == 5) => {

    }
    
    (2 * 2 == 4) => {

    }

    (3 >= 3) => {

    }
  end

  buz = [1,2,3,4]

  if buz.len and (buz.1 == 2 or buz.-1 == 4) {
    .log "hello ${buz[1]}"
  }

  contextualize one, fun {

  }


  mod helper {
    fun zero?(0) {
      true
    }

    fun zero?(x) when x > 1 {
      false
    }
  }
  
  .log helper.zero? 2

  buz = [1,2,3,4]

  if buz.len and (buz.1 == 2 or buz.-1 == 4) {
    .error "hello ${buz[1]}"
  }

  const helper = Object.freeze({
    isZero: (x) => {
      if (x === 0) {
        return true
      }

      if (x > 1) {
        return false
      }
    }
  })

  console.log(helper.isZero(2))

  const buz = [1,2,3,4]

  if(buz.length && (buz[1] === 2 || bus[-1] === 4 )) {
    console.error(`hello ${buz[1]}`)
  }
}
