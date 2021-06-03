package main

import (
	"net/http"

	"context"

	"github.com/gin-gonic/gin"

	"go.mongodb.org/mongo-driver/mongo"
	"go.mongodb.org/mongo-driver/mongo/options"
)

type Task struct {
	Title       string   `form:"title" json:"title" xml:"title"  binding:"required"`
	Id          string   `form:"id" json:"id" xml:"id"  binding:"required"`
	Date        string   `form:"date" json:"date" xml:"date"  binding:"required"`
	Creator     string   `form:"creator" json:"creator" xml:"creator"  binding:"required"`
	Asigned     string   `form:"asigned" json:"asigned" xml:"asigned"  binding:"required"`
	Description string   `form:"description" json:"description" xml:"description"  binding:"required"`
	Text        string   `form:"text" json:"text" xml:"text"  binding:"required"`
	Labels      []string `form:"labels" json:"labels" xml:"labels"  binding:"required"`
	Create_date string   `form:"create_date" json:"create_date" xml:"create_date"  binding:"required"`
	State       string   `form:"state" json:"state" xml:"state"  binding:"required"`
}

func list_task(c *gin.Context) {

	var filter interface{}
	name, _ := Client.ListDatabases(ctx, filter)
	c.String(http.StatusOK, "list_task %s", name)
}

func read_task(c *gin.Context) {
	name := c.Param("name")
	c.String(http.StatusOK, "read_task %s", name)
}

func create_task(c *gin.Context) {
	name := c.Param("name")
	c.String(http.StatusOK, "create_task %s", name)
}

func update_task(c *gin.Context) {
	name := c.Param("name")
	c.String(http.StatusOK, "update_task %s", name)
}

func delete_task(c *gin.Context) {
	name := c.Param("name")
	c.String(http.StatusOK, "delete_task %s", name)
}

var collection *mongo.Collection
var ctx = context.TODO()
var Client *mongo.Client

func init() {
	var appName *string
	appNameTmp := "athene-list_task"
	appName = &appNameTmp
	clientOptions := options.Client().ApplyURI("mongodb://root:example@localhost:27017")
	clientOptions.AppName(appName)

	Client, _ = mongo.Connect(ctx, clientOptions)

}

func main() {

	//gin.SetMode(gin.ReleaseMode)
	router := gin.Default()

	router.GET("/", list_task)

	// got task_id
	router.GET("/:task_id", read_task)
	router.POST("/:task_id", create_task)
	router.PUT("/:task_id", update_task)
	router.DELETE("/:task_id", delete_task)

	router.Run(":8080")
}
