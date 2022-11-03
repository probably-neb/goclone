package main

import (
	"encoding/json"
	"errors"
	"fmt"
	// "io/fs"
	"log"
	"os"
	"path/filepath"
)

const DB_PATH = "./db.json"
const DB_SIZEI = 128

func load_db() map[string]string {
   // TODO: return err instead of log.Fatal
   var db map[string]string
   // create db if it doesn't exist
   if _, err := os.Stat(DB_PATH); errors.Is(err, os.ErrNotExist) {
      os.Create(DB_PATH)
      if abs, err := filepath.Abs(DB_PATH); err == nil {
         fmt.Printf("creating db file: %s\n", abs)
         db := make(map[string]string, DB_SIZEI)
         return db
      } else {
         log.Fatal(err)
      }
   } else {
      if data, err := os.ReadFile(DB_PATH); err == nil {
         if json_err := json.Unmarshal(data, &db); json_err != nil {
            log.Fatal(json_err)
         }
      } else {
         log.Fatal(err)
      }
   }
   // fmt.Printf("I am: %s\n", os.Args[0])
   // fmt.Printf("db: %s\n", db)
   return db
}

func write_db(db map[string]string) error {
   data,err := json.MarshalIndent(db, "", "  ")
   fmt.Printf("db json: %s\n", data)
   if err == nil {
      err = os.WriteFile(DB_PATH, data, 0666)
   }
   return err
}

func main() {
   argc := len(os.Args)
   fmt.Printf("argc: %d\n", argc)
   if argc < 1 {
      return
   }

   db := load_db()
   if os.Args[1] == "add" {
      if argc != 4 {
         println("USAGE: goclone add local/path remote:path")
      }
      // TODO: check input
      local_path, err := filepath.Abs(os.Args[2])
      if err != nil {
         log.Fatal(err)
      }
      db[local_path] = os.Args[3]
      fmt.Printf("db: %s\n", db)
   }
   err := write_db(db)
   if err != nil {
      log.Fatal(err)
   }
}
