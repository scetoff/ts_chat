'Imports MySql.Data.MySqlClient
'Public Class frm_ViewPrivateMessage
'    Private IsFormBeingDragged As Boolean = False
'    Private MouseDownX As Integer
'    Private MouseDownY As Integer

'    Private IsFormBeingDraged As Boolean = False
'    Private MouseDovnX As Integer
'    Private MouseDovnY As Integer
'    Private Sub frm_auth_MouseDown(ByVal sender As Object, ByVal e As MouseEventArgs) Handles MyBase.MouseDown
'        If e.Button = MouseButtons.Left Then
'            IsFormBeingDragged = True
'            MouseDownX = e.X
'            MouseDownY = e.Y
'        End If
'    End Sub

'    Private Sub frm_auth_main_MouseUp(ByVal sender As Object, ByVal e As MouseEventArgs) Handles MyBase.MouseUp
'        If e.Button = MouseButtons.Left Then
'            IsFormBeingDragged = False
'        End If
'    End Sub

'    Private Sub frm_auth_MouseMove(ByVal sender As Object, ByVal e As MouseEventArgs) Handles MyBase.MouseMove
'        If IsFormBeingDragged Then
'            Dim temp As Point = New Point()

'            temp.X = Me.Location.X + (e.X - MouseDownX)
'            temp.Y = Me.Location.Y + (e.Y - MouseDownY)
'            Me.Location = temp
'            temp = Nothing
'        End If
'    End Sub


'    Private Sub Stringer_MouseDown(ByVal sender As Object, ByVal e As System.Windows.Forms.MouseEventArgs) Handles Stringer.MouseDown
'        If e.Button = MouseButtons.Left Then
'            IsFormBeingDraged = True
'            MouseDovnX = e.X
'            MouseDovnY = e.Y
'        End If
'    End Sub

'    Private Sub Stringer_MouseMove(ByVal sender As Object, ByVal e As System.Windows.Forms.MouseEventArgs) Handles Stringer.MouseMove
'        If IsFormBeingDraged Then
'            Dim pemp As Point = New Point()
'            pemp.X = Me.Location.X + (e.X - MouseDownX)
'            pemp.Y = Me.Location.Y + (e.Y - MouseDownY)
'            Me.Location = pemp
'            pemp = Nothing
'        End If
'    End Sub

'    Private Sub Stringer_MouseUp(ByVal sender As Object, ByVal e As System.Windows.Forms.MouseEventArgs) Handles Stringer.MouseUp
'        If e.Button = MouseButtons.Left Then
'            IsFormBeingDraged = False
'        End If
'    End Sub

'    Private Sub trei_Click(ByVal sender As System.Object, ByVal e As System.EventArgs) Handles trei.Click
'        Me.WindowState = FormWindowState.Minimized
'    End Sub

'    Private Sub EndIt_Click(ByVal sender As System.Object, ByVal e As System.EventArgs) Handles EndIt.Click
'        Me.Close()
'    End Sub

'    Private Sub frm_ViewPrivateMessage_Load(ByVal sender As System.Object, ByVal e As System.EventArgs) Handles MyBase.Load
'        Me.Text += " - " & My.Settings.FirstName
'        ConTempOpen()
'        Dim cmdreadmsgs As New MySqlCommand("SELECT * FROM tb_personal WHERE username='" & My.Settings.Username & "'", ConTemp)
'        Dim msgreader As MySqlDataReader

'        msgreader = cmdreadmsgs.ExecuteReader
'        Dim index As Integer = 0
'        While msgreader.Read()
'            lstMessages.Items.Add(msgreader(2).ToString).SubItems.Add(msgreader(3).ToString)
'            If msgreader(5).ToString = "0" Then

'                lstMessages.Items.Item(index).Font = New Font(lstMessages.Font.FontFamily, lstMessages.Font.Size, FontStyle.Bold)
'            End If
'            messages(index) = msgreader(4).ToString
'            index += 1
'        End While
'        msgreader.Close()
'        ConTemp.Close()

'    End Sub
'End Class