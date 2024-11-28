from airflow.utils.context import context

# Deprecated context variables (these should trigger linting errors)
execution_date = context["execution_date"]
next_ds = context["next_ds"]
next_ds_nodash = context["next_ds_nodash"]
next_execution_date = context["next_execution_date"]
prev_ds = context["prev_ds"]
prev_ds_nodash = context["prev_ds_nodash"]
prev_execution_date = context["prev_execution_date"]
prev_execution_date_success = context["prev_execution_date_success"]
tomorrow_ds = context["tomorrow_ds"]
yesterday_ds = context["yesterday_ds"]
yesterday_ds_nodash = context["yesterday_ds_nodash"]

# Valid usage (these should not trigger linting errors)
valid_variable = context["data_interval"]
